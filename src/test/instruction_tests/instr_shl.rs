use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 225, 72], OperandSize::Word)
}

#[test]
fn shl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 37, 90], OperandSize::Word)
}

#[test]
fn shl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 225, 22], OperandSize::Dword)
}

#[test]
fn shl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 246, 94], OperandSize::Dword)
}

#[test]
fn shl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 22], OperandSize::Qword)
}

#[test]
fn shl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(RSI, Two, 1817529596, Some(OperandSize::Byte), None)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 117, 252, 76, 85, 108, 17], OperandSize::Qword)
}

#[test]
fn shl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 51], OperandSize::Qword)
}

#[test]
fn shl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 86, 47], OperandSize::Qword)
}

#[test]
fn shl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DI)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 231, 96], OperandSize::Word)
}

#[test]
fn shl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(BX, 149, Some(OperandSize::Word), None)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 167, 149, 0, 64], OperandSize::Word)
}

#[test]
fn shl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DX)), operand2: Some(Literal8(122)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 226, 122], OperandSize::Dword)
}

#[test]
fn shl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 36, 201, 21], OperandSize::Dword)
}

#[test]
fn shl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BP)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 229, 107], OperandSize::Qword)
}

#[test]
fn shl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 39, 84], OperandSize::Qword)
}

#[test]
fn shl_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EDX)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 226, 80], OperandSize::Word)
}

#[test]
fn shl_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 16290, Some(OperandSize::Dword), None)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 161, 162, 63, 16], OperandSize::Word)
}

#[test]
fn shl_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 230, 86], OperandSize::Dword)
}

#[test]
fn shl_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 982333254, Some(OperandSize::Dword), None)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 164, 137, 70, 55, 141, 58, 48], OperandSize::Dword)
}

#[test]
fn shl_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 229, 63], OperandSize::Qword)
}

#[test]
fn shl_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 1354288806, Some(OperandSize::Dword), None)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 164, 154, 166, 206, 184, 80, 109], OperandSize::Qword)
}

#[test]
fn shl_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(RDX)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 226, 44], OperandSize::Qword)
}

#[test]
fn shl_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(RCX, 112681333, Some(OperandSize::Qword), None)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 161, 117, 97, 183, 6, 10], OperandSize::Qword)
}

#[test]
fn shl_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 226], OperandSize::Word)
}

#[test]
fn shl_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 23770, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 163, 218, 92], OperandSize::Word)
}

#[test]
fn shl_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 225], OperandSize::Dword)
}

#[test]
fn shl_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 32], OperandSize::Dword)
}

#[test]
fn shl_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Qword)
}

#[test]
fn shl_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 32], OperandSize::Qword)
}

#[test]
fn shl_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Qword)
}

#[test]
fn shl_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 36, 87], OperandSize::Qword)
}

#[test]
fn shl_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 230], OperandSize::Word)
}

#[test]
fn shl_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 118, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 97, 118], OperandSize::Word)
}

#[test]
fn shl_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 231], OperandSize::Dword)
}

#[test]
fn shl_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 586560599, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 164, 136, 87, 52, 246, 34], OperandSize::Dword)
}

#[test]
fn shl_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 225], OperandSize::Qword)
}

#[test]
fn shl_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 693457010, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 164, 95, 114, 80, 85, 41], OperandSize::Qword)
}

#[test]
fn shl_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 226], OperandSize::Word)
}

#[test]
fn shl_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(SI, 173, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 164, 173, 0], OperandSize::Word)
}

#[test]
fn shl_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 230], OperandSize::Dword)
}

#[test]
fn shl_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 944704132, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 36, 221, 132, 10, 79, 56], OperandSize::Dword)
}

#[test]
fn shl_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 227], OperandSize::Qword)
}

#[test]
fn shl_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(RSI, 1808891303, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 166, 167, 125, 209, 107], OperandSize::Qword)
}

#[test]
fn shl_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(RCX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 225], OperandSize::Qword)
}

#[test]
fn shl_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(RBX, 988930619, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 163, 59, 226, 241, 58], OperandSize::Qword)
}

#[test]
fn shl_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 227], OperandSize::Word)
}

#[test]
fn shl_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(BP, 25900, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 166, 44, 101], OperandSize::Word)
}

#[test]
fn shl_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Dword)
}

#[test]
fn shl_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1042038522, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 164, 112, 250, 62, 28, 62], OperandSize::Dword)
}

#[test]
fn shl_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 226], OperandSize::Qword)
}

#[test]
fn shl_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 918045121, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 164, 179, 193, 65, 184, 54], OperandSize::Qword)
}

#[test]
fn shl_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 226], OperandSize::Qword)
}

#[test]
fn shl_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 35], OperandSize::Qword)
}

#[test]
fn shl_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 225], OperandSize::Word)
}

#[test]
fn shl_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(DI, 31009, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 165, 33, 121], OperandSize::Word)
}

#[test]
fn shl_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 225], OperandSize::Dword)
}

#[test]
fn shl_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 102378811, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 164, 115, 59, 45, 26, 6], OperandSize::Dword)
}

#[test]
fn shl_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 225], OperandSize::Qword)
}

#[test]
fn shl_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 947285015, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 164, 127, 23, 108, 118, 56], OperandSize::Qword)
}

#[test]
fn shl_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 229], OperandSize::Word)
}

#[test]
fn shl_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(BP, 30, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 102, 30], OperandSize::Word)
}

#[test]
fn shl_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 227], OperandSize::Dword)
}

#[test]
fn shl_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 39], OperandSize::Dword)
}

#[test]
fn shl_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 225], OperandSize::Qword)
}

#[test]
fn shl_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(RDI, Two, 669860805, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 36, 125, 197, 67, 237, 39], OperandSize::Qword)
}

#[test]
fn shl_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(RBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 229], OperandSize::Qword)
}

#[test]
fn shl_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 586882908, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 164, 195, 92, 31, 251, 34], OperandSize::Qword)
}

