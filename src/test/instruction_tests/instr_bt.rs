use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 233], OperandSize::Word)
}

#[test]
fn bt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 35], OperandSize::Word)
}

#[test]
fn bt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 231], OperandSize::Dword)
}

#[test]
fn bt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 26], OperandSize::Dword)
}

#[test]
fn bt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 249], OperandSize::Qword)
}

#[test]
fn bt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1484583389, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 140, 182, 221, 241, 124, 88], OperandSize::Qword)
}

#[test]
fn bt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 229], OperandSize::Word)
}

#[test]
fn bt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 48], OperandSize::Word)
}

#[test]
fn bt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 207], OperandSize::Dword)
}

#[test]
fn bt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledDisplaced(EBX, Four, 1340134948, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 36, 157, 36, 214, 224, 79], OperandSize::Dword)
}

#[test]
fn bt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 241], OperandSize::Qword)
}

#[test]
fn bt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1958329501, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 148, 147, 157, 188, 185, 116], OperandSize::Qword)
}

#[test]
fn bt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(RSI)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 163, 206], OperandSize::Qword)
}

#[test]
fn bt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 163, 52, 120], OperandSize::Qword)
}

#[test]
fn bt_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(BP)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 229, 89], OperandSize::Word)
}

#[test]
fn bt_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 18383, Some(OperandSize::Word), None)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 161, 207, 71, 97], OperandSize::Word)
}

#[test]
fn bt_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(DI)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 231, 90], OperandSize::Dword)
}

#[test]
fn bt_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 727860413, Some(OperandSize::Word), None)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 164, 203, 189, 68, 98, 43, 47], OperandSize::Dword)
}

#[test]
fn bt_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(BP)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 229, 49], OperandSize::Qword)
}

#[test]
fn bt_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 38, 66], OperandSize::Qword)
}

#[test]
fn bt_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(ESP)), operand2: Some(Literal8(83)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 228, 83], OperandSize::Word)
}

#[test]
fn bt_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 18, Some(OperandSize::Dword), None)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 98, 18, 80], OperandSize::Word)
}

#[test]
fn bt_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EDI)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 231, 2], OperandSize::Dword)
}

#[test]
fn bt_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 39, 70], OperandSize::Dword)
}

#[test]
fn bt_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(ESI)), operand2: Some(Literal8(102)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 230, 102], OperandSize::Qword)
}

#[test]
fn bt_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 36, 222, 91], OperandSize::Qword)
}

#[test]
fn bt_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(RBP)), operand2: Some(Literal8(42)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 229, 42], OperandSize::Qword)
}

#[test]
fn bt_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 33, 29], OperandSize::Qword)
}

