use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 217, 67], OperandSize::Word)
}

#[test]
fn rcr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 218, Some(OperandSize::Byte), None)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 152, 218, 0, 24], OperandSize::Word)
}

#[test]
fn rcr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 218, 88], OperandSize::Dword)
}

#[test]
fn rcr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1183721837, Some(OperandSize::Byte), None)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 156, 217, 109, 41, 142, 70, 0], OperandSize::Dword)
}

#[test]
fn rcr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 217, 109], OperandSize::Qword)
}

#[test]
fn rcr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 24, 106], OperandSize::Qword)
}

#[test]
fn rcr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 219, 43], OperandSize::Qword)
}

#[test]
fn rcr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 28, 216, 21], OperandSize::Qword)
}

#[test]
fn rcr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(SP)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 220, 121], OperandSize::Word)
}

#[test]
fn rcr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 26, 109], OperandSize::Word)
}

#[test]
fn rcr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BX)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 219, 4], OperandSize::Dword)
}

#[test]
fn rcr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(EAX, Four, 709324884, Some(OperandSize::Word), None)), operand2: Some(Literal8(112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 28, 133, 84, 112, 71, 42, 112], OperandSize::Dword)
}

#[test]
fn rcr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(SP)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 220, 78], OperandSize::Qword)
}

#[test]
fn rcr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 28, 251, 16], OperandSize::Qword)
}

#[test]
fn rcr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 222, 85], OperandSize::Word)
}

#[test]
fn rcr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 29, 66], OperandSize::Word)
}

#[test]
fn rcr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 219, 44], OperandSize::Dword)
}

#[test]
fn rcr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 25, 3], OperandSize::Dword)
}

#[test]
fn rcr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 217, 111], OperandSize::Qword)
}

#[test]
fn rcr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1273736445, Some(OperandSize::Dword), None)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 156, 87, 253, 172, 235, 75, 78], OperandSize::Qword)
}

#[test]
fn rcr_21() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(RCX)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 217, 41], OperandSize::Qword)
}

#[test]
fn rcr_22() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(RDI, 290136194, Some(OperandSize::Qword), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 159, 130, 32, 75, 17, 92], OperandSize::Qword)
}

#[test]
fn rcr_23() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 219], OperandSize::Word)
}

#[test]
fn rcr_24() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 26], OperandSize::Word)
}

#[test]
fn rcr_25() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 219], OperandSize::Dword)
}

#[test]
fn rcr_26() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(EDI, Two, 914511026, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 28, 125, 178, 84, 130, 54], OperandSize::Dword)
}

#[test]
fn rcr_27() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 219], OperandSize::Qword)
}

#[test]
fn rcr_28() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 911103021, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 28, 213, 45, 84, 78, 54], OperandSize::Qword)
}

#[test]
fn rcr_29() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 218], OperandSize::Qword)
}

#[test]
fn rcr_30() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RDI, Four, 448139524, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 28, 189, 4, 17, 182, 26], OperandSize::Qword)
}

#[test]
fn rcr_31() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 219], OperandSize::Word)
}

#[test]
fn rcr_32() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 31], OperandSize::Word)
}

#[test]
fn rcr_33() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 218], OperandSize::Dword)
}

#[test]
fn rcr_34() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 26], OperandSize::Dword)
}

#[test]
fn rcr_35() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 220], OperandSize::Qword)
}

#[test]
fn rcr_36() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(RDI, 782318473, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 159, 137, 59, 161, 46], OperandSize::Qword)
}

#[test]
fn rcr_37() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 217], OperandSize::Word)
}

#[test]
fn rcr_38() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(BP, 121, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 94, 121], OperandSize::Word)
}

#[test]
fn rcr_39() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 217], OperandSize::Dword)
}

#[test]
fn rcr_40() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1961085341, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 28, 189, 157, 201, 227, 116], OperandSize::Dword)
}

#[test]
fn rcr_41() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 218], OperandSize::Qword)
}

#[test]
fn rcr_42() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 210984071, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 28, 221, 135, 92, 147, 12], OperandSize::Qword)
}

#[test]
fn rcr_43() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(RDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 218], OperandSize::Qword)
}

#[test]
fn rcr_44() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 28, 122], OperandSize::Qword)
}

#[test]
fn rcr_45() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 219], OperandSize::Word)
}

#[test]
fn rcr_46() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 26], OperandSize::Word)
}

#[test]
fn rcr_47() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 218], OperandSize::Dword)
}

#[test]
fn rcr_48() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 538615146, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 156, 137, 106, 157, 26, 32], OperandSize::Dword)
}

#[test]
fn rcr_49() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 219], OperandSize::Qword)
}

#[test]
fn rcr_50() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 30], OperandSize::Qword)
}

#[test]
fn rcr_51() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 219], OperandSize::Qword)
}

#[test]
fn rcr_52() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 31], OperandSize::Qword)
}

#[test]
fn rcr_53() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 221], OperandSize::Word)
}

#[test]
fn rcr_54() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 2365, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 155, 61, 9], OperandSize::Word)
}

#[test]
fn rcr_55() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 222], OperandSize::Dword)
}

#[test]
fn rcr_56() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 28, 158], OperandSize::Dword)
}

#[test]
fn rcr_57() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 223], OperandSize::Qword)
}

#[test]
fn rcr_58() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(RDX, 1286966249, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 154, 233, 139, 181, 76], OperandSize::Qword)
}

#[test]
fn rcr_59() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 220], OperandSize::Word)
}

#[test]
fn rcr_60() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 27], OperandSize::Word)
}

#[test]
fn rcr_61() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 219], OperandSize::Dword)
}

#[test]
fn rcr_62() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(EDI, 75365125, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 159, 5, 251, 125, 4], OperandSize::Dword)
}

#[test]
fn rcr_63() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 218], OperandSize::Qword)
}

#[test]
fn rcr_64() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1667257652, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 28, 189, 52, 85, 96, 99], OperandSize::Qword)
}

#[test]
fn rcr_65() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(RSI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 222], OperandSize::Qword)
}

#[test]
fn rcr_66() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 24], OperandSize::Qword)
}

