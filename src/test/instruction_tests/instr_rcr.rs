use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 217, 36], OperandSize::Word)
}

#[test]
fn rcr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 19, Some(OperandSize::Byte), None)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 88, 19, 117], OperandSize::Word)
}

#[test]
fn rcr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 219, 61], OperandSize::Dword)
}

#[test]
fn rcr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 24, 14], OperandSize::Dword)
}

#[test]
fn rcr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(102)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 217, 102], OperandSize::Qword)
}

#[test]
fn rcr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RSI, Four, 2054649939, Some(OperandSize::Byte), None)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 28, 181, 83, 120, 119, 122, 98], OperandSize::Qword)
}

#[test]
fn rcr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 217, 98], OperandSize::Qword)
}

#[test]
fn rcr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(RDI, 1089622221, Some(OperandSize::Byte), None)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 159, 205, 80, 242, 64, 103], OperandSize::Qword)
}

#[test]
fn rcr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(SP)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 220, 68], OperandSize::Word)
}

#[test]
fn rcr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 222, Some(OperandSize::Word), None)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 152, 222, 0, 62], OperandSize::Word)
}

#[test]
fn rcr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DX)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 218, 50], OperandSize::Dword)
}

#[test]
fn rcr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 713442772, Some(OperandSize::Word), None)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 156, 113, 212, 69, 134, 42, 14], OperandSize::Dword)
}

#[test]
fn rcr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DI)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 223, 78], OperandSize::Qword)
}

#[test]
fn rcr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 1701846007, Some(OperandSize::Word), None)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 156, 158, 247, 27, 112, 101, 14], OperandSize::Qword)
}

#[test]
fn rcr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 217, 60], OperandSize::Word)
}

#[test]
fn rcr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 26, 40], OperandSize::Word)
}

#[test]
fn rcr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 218, 34], OperandSize::Dword)
}

#[test]
fn rcr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 1668198053, Some(OperandSize::Dword), None)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 156, 201, 165, 174, 110, 99, 58], OperandSize::Dword)
}

#[test]
fn rcr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 219, 88], OperandSize::Qword)
}

#[test]
fn rcr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1343727830, Some(OperandSize::Dword), None)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 28, 85, 214, 168, 23, 80, 93], OperandSize::Qword)
}

#[test]
fn rcr_21() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(RBX)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 219, 6], OperandSize::Qword)
}

#[test]
fn rcr_22() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(RAX, 1989162255, Some(OperandSize::Qword), None)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 152, 15, 53, 144, 118, 51], OperandSize::Qword)
}

#[test]
fn rcr_23() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 217], OperandSize::Word)
}

#[test]
fn rcr_24() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 22106, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 153, 90, 86], OperandSize::Word)
}

#[test]
fn rcr_25() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 218], OperandSize::Dword)
}

#[test]
fn rcr_26() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(EAX, Two, 236465438, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 28, 69, 30, 45, 24, 14], OperandSize::Dword)
}

#[test]
fn rcr_27() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 219], OperandSize::Qword)
}

#[test]
fn rcr_28() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 28, 113], OperandSize::Qword)
}

#[test]
fn rcr_29() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 219], OperandSize::Qword)
}

#[test]
fn rcr_30() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RDI, Two, 529829055, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 28, 125, 191, 140, 148, 31], OperandSize::Qword)
}

#[test]
fn rcr_31() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 221], OperandSize::Word)
}

#[test]
fn rcr_32() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(BX, 7243, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 159, 75, 28], OperandSize::Word)
}

#[test]
fn rcr_33() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 218], OperandSize::Dword)
}

#[test]
fn rcr_34() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(EBX, 4695937, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 155, 129, 167, 71, 0], OperandSize::Dword)
}

#[test]
fn rcr_35() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 218], OperandSize::Qword)
}

#[test]
fn rcr_36() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 28, 186], OperandSize::Qword)
}

#[test]
fn rcr_37() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 220], OperandSize::Word)
}

#[test]
fn rcr_38() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 28], OperandSize::Word)
}

#[test]
fn rcr_39() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 217], OperandSize::Dword)
}

#[test]
fn rcr_40() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(ESI, 732091946, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 158, 42, 214, 162, 43], OperandSize::Dword)
}

#[test]
fn rcr_41() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 220], OperandSize::Qword)
}

#[test]
fn rcr_42() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 864539688, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 156, 128, 40, 212, 135, 51], OperandSize::Qword)
}

#[test]
fn rcr_43() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(RDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 223], OperandSize::Qword)
}

#[test]
fn rcr_44() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RDI, Two, 2136773209, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 28, 125, 89, 146, 92, 127], OperandSize::Qword)
}

#[test]
fn rcr_45() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 219], OperandSize::Word)
}

#[test]
fn rcr_46() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 30651, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 153, 187, 119], OperandSize::Word)
}

#[test]
fn rcr_47() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 218], OperandSize::Dword)
}

#[test]
fn rcr_48() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(EBX, Four, 981689541, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 28, 157, 197, 100, 131, 58], OperandSize::Dword)
}

#[test]
fn rcr_49() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 218], OperandSize::Qword)
}

#[test]
fn rcr_50() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RDX, Two, 2034463569, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 28, 85, 81, 115, 67, 121], OperandSize::Qword)
}

#[test]
fn rcr_51() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 219], OperandSize::Qword)
}

#[test]
fn rcr_52() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RSI, Four, 199469247, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 28, 181, 191, 168, 227, 11], OperandSize::Qword)
}

#[test]
fn rcr_53() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 220], OperandSize::Word)
}

#[test]
fn rcr_54() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 6, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 89, 6], OperandSize::Word)
}

#[test]
fn rcr_55() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 217], OperandSize::Dword)
}

#[test]
fn rcr_56() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 28, 142], OperandSize::Dword)
}

#[test]
fn rcr_57() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 217], OperandSize::Qword)
}

#[test]
fn rcr_58() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1955678898, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 28, 221, 178, 74, 145, 116], OperandSize::Qword)
}

#[test]
fn rcr_59() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 218], OperandSize::Word)
}

#[test]
fn rcr_60() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(DI, 18477, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 157, 45, 72], OperandSize::Word)
}

#[test]
fn rcr_61() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 220], OperandSize::Dword)
}

#[test]
fn rcr_62() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1782645305, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 28, 69, 57, 2, 65, 106], OperandSize::Dword)
}

#[test]
fn rcr_63() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 218], OperandSize::Qword)
}

#[test]
fn rcr_64() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 30], OperandSize::Qword)
}

#[test]
fn rcr_65() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(RDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 223], OperandSize::Qword)
}

#[test]
fn rcr_66() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(RSI, 1754788522, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 158, 170, 242, 151, 104], OperandSize::Qword)
}

