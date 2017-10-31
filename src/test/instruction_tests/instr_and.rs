use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn and_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 219], OperandSize::Word)
}

#[test]
fn and_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 65, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 88, 65], OperandSize::Word)
}

#[test]
fn and_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 219], OperandSize::Dword)
}

#[test]
fn and_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 8], OperandSize::Dword)
}

#[test]
fn and_5() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 211], OperandSize::Qword)
}

#[test]
fn and_6() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 28, 126], OperandSize::Qword)
}

#[test]
fn and_7() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 209], OperandSize::Qword)
}

#[test]
fn and_8() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 12, 202], OperandSize::Qword)
}

#[test]
fn and_9() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 227], OperandSize::Word)
}

#[test]
fn and_10() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(BP, 204, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 150, 204, 0], OperandSize::Word)
}

#[test]
fn and_11() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 246], OperandSize::Dword)
}

#[test]
fn and_12() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 49], OperandSize::Dword)
}

#[test]
fn and_13() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 218], OperandSize::Qword)
}

#[test]
fn and_14() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 52, 207], OperandSize::Qword)
}

#[test]
fn and_15() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 227], OperandSize::Word)
}

#[test]
fn and_16() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 48], OperandSize::Word)
}

#[test]
fn and_17() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 221], OperandSize::Dword)
}

#[test]
fn and_18() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 1297466816, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 20, 253, 192, 197, 85, 77], OperandSize::Dword)
}

#[test]
fn and_19() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 236], OperandSize::Qword)
}

#[test]
fn and_20() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 22], OperandSize::Qword)
}

#[test]
fn and_21() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 221], OperandSize::Qword)
}

#[test]
fn and_22() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 685862254, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 36, 205, 110, 109, 225, 40], OperandSize::Qword)
}

#[test]
fn and_23() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 202], OperandSize::Word)
}

#[test]
fn and_24() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 25719, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 137, 119, 100], OperandSize::Word)
}

#[test]
fn and_25() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 203], OperandSize::Dword)
}

#[test]
fn and_26() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 12, 121], OperandSize::Dword)
}

#[test]
fn and_27() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 201], OperandSize::Qword)
}

#[test]
fn and_28() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 2042333041, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 156, 151, 113, 135, 187, 121], OperandSize::Qword)
}

#[test]
fn and_29() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 203], OperandSize::Qword)
}

#[test]
fn and_30() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(RAX, 1363890629, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 136, 197, 81, 75, 81], OperandSize::Qword)
}

#[test]
fn and_31() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 209], OperandSize::Word)
}

#[test]
fn and_32() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 19399, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 176, 199, 75], OperandSize::Word)
}

#[test]
fn and_33() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 217], OperandSize::Dword)
}

#[test]
fn and_34() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 722666418, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 52, 149, 178, 3, 19, 43], OperandSize::Dword)
}

#[test]
fn and_35() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 227], OperandSize::Qword)
}

#[test]
fn and_36() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1403862274, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 20, 77, 2, 61, 173, 83], OperandSize::Qword)
}

#[test]
fn and_37() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 231], OperandSize::Word)
}

#[test]
fn and_38() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(SI, 205, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 188, 205, 0], OperandSize::Word)
}

#[test]
fn and_39() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 246], OperandSize::Dword)
}

#[test]
fn and_40() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1879745912, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 20, 253, 120, 165, 10, 112], OperandSize::Dword)
}

#[test]
fn and_41() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 219], OperandSize::Qword)
}

#[test]
fn and_42() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 123598303, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 180, 79, 223, 245, 93, 7], OperandSize::Qword)
}

#[test]
fn and_43() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 222], OperandSize::Qword)
}

#[test]
fn and_44() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 338575291, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 35, 148, 88, 187, 63, 46, 20], OperandSize::Qword)
}

#[test]
fn and_45() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(119)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 119], OperandSize::Word)
}

#[test]
fn and_46() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 75], OperandSize::Dword)
}

#[test]
fn and_47() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 71], OperandSize::Qword)
}

#[test]
fn and_48() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(29610)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 170, 115], OperandSize::Word)
}

#[test]
fn and_49() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(16333)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 205, 63], OperandSize::Dword)
}

#[test]
fn and_50() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(30898)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 178, 120], OperandSize::Qword)
}

#[test]
fn and_51() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1286009920)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 64, 244, 166, 76], OperandSize::Word)
}

#[test]
fn and_52() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(2008431604)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 244, 59, 182, 119], OperandSize::Dword)
}

#[test]
fn and_53() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1432537315)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 227, 200, 98, 85], OperandSize::Qword)
}

#[test]
fn and_54() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1056338287)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 37, 111, 113, 246, 62], OperandSize::Qword)
}

#[test]
fn and_55() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 226, 58], OperandSize::Word)
}

#[test]
fn and_56() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(BX, 20000, Some(OperandSize::Byte), None)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 167, 32, 78, 16], OperandSize::Word)
}

#[test]
fn and_57() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 225, 86], OperandSize::Dword)
}

#[test]
fn and_58() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 36, 215, 120], OperandSize::Dword)
}

#[test]
fn and_59() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 225, 58], OperandSize::Qword)
}

#[test]
fn and_60() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 492765579, Some(OperandSize::Byte), None)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 164, 118, 139, 1, 95, 29, 104], OperandSize::Qword)
}

#[test]
fn and_61() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 227, 113], OperandSize::Qword)
}

#[test]
fn and_62() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1078396275, Some(OperandSize::Byte), None)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 164, 209, 115, 5, 71, 64, 78], OperandSize::Qword)
}

#[test]
fn and_63() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BX)), operand2: Some(Literal16(9366)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 227, 150, 36], OperandSize::Word)
}

#[test]
fn and_64() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Memory(17818, Some(OperandSize::Word), None)), operand2: Some(Literal16(3224)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 38, 154, 69, 152, 12], OperandSize::Word)
}

#[test]
fn and_65() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BP)), operand2: Some(Literal16(8634)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 229, 186, 33], OperandSize::Dword)
}

#[test]
fn and_66() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(EAX, 1640057460, Some(OperandSize::Word), None)), operand2: Some(Literal16(30047)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 160, 116, 74, 193, 97, 95, 117], OperandSize::Dword)
}

#[test]
fn and_67() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(Literal16(21486)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 230, 238, 83], OperandSize::Qword)
}

#[test]
fn and_68() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 1804021687, Some(OperandSize::Word), None)), operand2: Some(Literal16(18002)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 164, 91, 183, 47, 135, 107, 82, 70], OperandSize::Qword)
}

#[test]
fn and_69() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Literal32(2048067881)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 227, 41, 9, 19, 122], OperandSize::Word)
}

#[test]
fn and_70() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 29803, Some(OperandSize::Dword), None)), operand2: Some(Literal32(312286391)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 163, 107, 116, 183, 28, 157, 18], OperandSize::Word)
}

#[test]
fn and_71() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBP)), operand2: Some(Literal32(787535313)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 229, 209, 213, 240, 46], OperandSize::Dword)
}

#[test]
fn and_72() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal32(115871714)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 36, 202, 226, 15, 232, 6], OperandSize::Dword)
}

#[test]
fn and_73() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Literal32(484950730)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 226, 202, 194, 231, 28], OperandSize::Qword)
}

#[test]
fn and_74() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(RDI, 64155398, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1726751913)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 167, 6, 239, 210, 3, 169, 36, 236, 102], OperandSize::Qword)
}

#[test]
fn and_75() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RCX)), operand2: Some(Literal32(894433432)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 225, 152, 248, 79, 53], OperandSize::Qword)
}

#[test]
fn and_76() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1274968177)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 36, 251, 113, 120, 254, 75], OperandSize::Qword)
}

#[test]
fn and_77() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DI)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 231, 65], OperandSize::Word)
}

#[test]
fn and_78() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(DI, 176, Some(OperandSize::Word), None)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 165, 176, 0, 101], OperandSize::Word)
}

#[test]
fn and_79() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CX)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 225, 105], OperandSize::Dword)
}

#[test]
fn and_80() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 35, 103], OperandSize::Dword)
}

#[test]
fn and_81() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BX)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 227, 39], OperandSize::Qword)
}

#[test]
fn and_82() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(RSI, Two, 932510173, Some(OperandSize::Word), None)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 36, 117, 221, 249, 148, 55, 61], OperandSize::Qword)
}

#[test]
fn and_83() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBP)), operand2: Some(Literal8(111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 229, 111], OperandSize::Word)
}

#[test]
fn and_84() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 221, Some(OperandSize::Dword), None)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 161, 221, 0, 16], OperandSize::Word)
}

#[test]
fn and_85() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 227, 17], OperandSize::Dword)
}

#[test]
fn and_86() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 36, 65, 84], OperandSize::Dword)
}

#[test]
fn and_87() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDI)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 231, 68], OperandSize::Qword)
}

#[test]
fn and_88() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 35, 49], OperandSize::Qword)
}

#[test]
fn and_89() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RCX)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 225, 63], OperandSize::Qword)
}

#[test]
fn and_90() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 163245294, Some(OperandSize::Qword), None)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 164, 193, 238, 236, 186, 9, 50], OperandSize::Qword)
}

