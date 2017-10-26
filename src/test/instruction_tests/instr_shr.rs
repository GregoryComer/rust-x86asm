use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CL)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 233, 84], OperandSize::Word)
}

#[test]
fn shr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 233, Some(OperandSize::Byte), None)), operand2: Some(Literal8(119)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 170, 233, 0, 119], OperandSize::Word)
}

#[test]
fn shr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 234, 11], OperandSize::Dword)
}

#[test]
fn shr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 43, 5], OperandSize::Dword)
}

#[test]
fn shr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BL)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 235, 79], OperandSize::Qword)
}

#[test]
fn shr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RBX, 1710061146, Some(OperandSize::Byte), None)), operand2: Some(Literal8(102)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 171, 90, 118, 237, 101, 102], OperandSize::Qword)
}

#[test]
fn shr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CL)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 233, 104], OperandSize::Qword)
}

#[test]
fn shr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1892780751, Some(OperandSize::Byte), None)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 44, 189, 207, 138, 209, 112, 4], OperandSize::Qword)
}

#[test]
fn shr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BX)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 235, 67], OperandSize::Word)
}

#[test]
fn shr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 41, 48], OperandSize::Word)
}

#[test]
fn shr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DI)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 239, 59], OperandSize::Dword)
}

#[test]
fn shr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(ECX, 525889157, Some(OperandSize::Word), None)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 169, 133, 110, 88, 31, 85], OperandSize::Dword)
}

#[test]
fn shr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BX)), operand2: Some(Literal8(102)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 235, 102], OperandSize::Qword)
}

#[test]
fn shr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(RCX, Four, 479778303, Some(OperandSize::Word), None)), operand2: Some(Literal8(118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 44, 141, 255, 213, 152, 28, 118], OperandSize::Qword)
}

#[test]
fn shr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 237, 25], OperandSize::Word)
}

#[test]
fn shr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 161, Some(OperandSize::Dword), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 169, 161, 0, 71], OperandSize::Word)
}

#[test]
fn shr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 237, 28], OperandSize::Dword)
}

#[test]
fn shr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(EDX, 913117791, Some(OperandSize::Dword), None)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 170, 95, 18, 109, 54, 126], OperandSize::Dword)
}

#[test]
fn shr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 235, 25], OperandSize::Qword)
}

#[test]
fn shr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 44, 210, 75], OperandSize::Qword)
}

#[test]
fn shr_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(RBP)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 237, 2], OperandSize::Qword)
}

#[test]
fn shr_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 42, 65], OperandSize::Qword)
}

#[test]
fn shr_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 234], OperandSize::Word)
}

#[test]
fn shr_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 43], OperandSize::Word)
}

#[test]
fn shr_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 234], OperandSize::Dword)
}

#[test]
fn shr_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1264163368, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 44, 77, 40, 154, 89, 75], OperandSize::Dword)
}

#[test]
fn shr_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 233], OperandSize::Qword)
}

#[test]
fn shr_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 44, 179], OperandSize::Qword)
}

#[test]
fn shr_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 234], OperandSize::Qword)
}

#[test]
fn shr_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RDI, 373279124, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 175, 148, 201, 63, 22], OperandSize::Qword)
}

#[test]
fn shr_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 239], OperandSize::Word)
}

#[test]
fn shr_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 216, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 171, 216, 0], OperandSize::Word)
}

#[test]
fn shr_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 239], OperandSize::Dword)
}

#[test]
fn shr_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 43], OperandSize::Dword)
}

#[test]
fn shr_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 234], OperandSize::Qword)
}

#[test]
fn shr_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(RSI, Four, 149854981, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 44, 181, 5, 155, 238, 8], OperandSize::Qword)
}

#[test]
fn shr_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 236], OperandSize::Word)
}

#[test]
fn shr_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 45], OperandSize::Word)
}

#[test]
fn shr_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 237], OperandSize::Dword)
}

#[test]
fn shr_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 44, 184], OperandSize::Dword)
}

#[test]
fn shr_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 239], OperandSize::Qword)
}

#[test]
fn shr_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RDX, 999113073, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 170, 113, 65, 141, 59], OperandSize::Qword)
}

#[test]
fn shr_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(RBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 235], OperandSize::Qword)
}

#[test]
fn shr_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RAX, 1000596417, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 168, 193, 227, 163, 59], OperandSize::Qword)
}

#[test]
fn shr_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 235], OperandSize::Word)
}

#[test]
fn shr_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 3569, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 168, 241, 13], OperandSize::Word)
}

#[test]
fn shr_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 234], OperandSize::Dword)
}

#[test]
fn shr_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1871028040, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 172, 214, 72, 159, 133, 111], OperandSize::Dword)
}

#[test]
fn shr_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 233], OperandSize::Qword)
}

#[test]
fn shr_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RAX, 1246384082, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 168, 210, 79, 74, 74], OperandSize::Qword)
}

#[test]
fn shr_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 233], OperandSize::Qword)
}

#[test]
fn shr_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 44, 136], OperandSize::Qword)
}

#[test]
fn shr_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 237], OperandSize::Word)
}

#[test]
fn shr_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 19370, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 170, 170, 75], OperandSize::Word)
}

#[test]
fn shr_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 239], OperandSize::Dword)
}

#[test]
fn shr_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 44, 130], OperandSize::Dword)
}

#[test]
fn shr_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 233], OperandSize::Qword)
}

#[test]
fn shr_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RAX, 631465471, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 168, 255, 101, 163, 37], OperandSize::Qword)
}

#[test]
fn shr_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 236], OperandSize::Word)
}

#[test]
fn shr_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(SI, 104, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 108, 104], OperandSize::Word)
}

#[test]
fn shr_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 233], OperandSize::Dword)
}

#[test]
fn shr_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 795322150, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 172, 241, 38, 167, 103, 47], OperandSize::Dword)
}

#[test]
fn shr_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 233], OperandSize::Qword)
}

#[test]
fn shr_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 44, 121], OperandSize::Qword)
}

#[test]
fn shr_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(RBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 237], OperandSize::Qword)
}

#[test]
fn shr_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 40], OperandSize::Qword)
}

