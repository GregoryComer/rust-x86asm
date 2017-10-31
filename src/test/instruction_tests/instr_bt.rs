use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 242], OperandSize::Word)
}

#[test]
fn bt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectDisplaced(SI, 11, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 92, 11], OperandSize::Word)
}

#[test]
fn bt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 217], OperandSize::Dword)
}

#[test]
fn bt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectDisplaced(ESI, 453927434, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 142, 10, 98, 14, 27], OperandSize::Dword)
}

#[test]
fn bt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 252], OperandSize::Qword)
}

#[test]
fn bt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 582367269, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 60, 245, 37, 56, 182, 34], OperandSize::Qword)
}

#[test]
fn bt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 211], OperandSize::Word)
}

#[test]
fn bt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectDisplaced(BP, 100, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 94, 100], OperandSize::Word)
}

#[test]
fn bt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 222], OperandSize::Dword)
}

#[test]
fn bt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectDisplaced(EDI, 1452412999, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 143, 71, 16, 146, 86], OperandSize::Dword)
}

#[test]
fn bt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 210], OperandSize::Qword)
}

#[test]
fn bt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 1766583190, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 188, 207, 150, 235, 75, 105], OperandSize::Qword)
}

#[test]
fn bt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 163, 222], OperandSize::Qword)
}

#[test]
fn bt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 163, 58], OperandSize::Qword)
}

#[test]
fn bt_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(DX)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 226, 93], OperandSize::Word)
}

#[test]
fn bt_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 19202, Some(OperandSize::Word), None)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 162, 2, 75, 36], OperandSize::Word)
}

#[test]
fn bt_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(BP)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 229, 76], OperandSize::Dword)
}

#[test]
fn bt_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1425071094, Some(OperandSize::Word), None)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 36, 189, 246, 219, 240, 84, 6], OperandSize::Dword)
}

#[test]
fn bt_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(SI)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 230, 32], OperandSize::Qword)
}

#[test]
fn bt_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 36, 80, 52], OperandSize::Qword)
}

#[test]
fn bt_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EDX)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 226, 87], OperandSize::Word)
}

#[test]
fn bt_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 37, 58], OperandSize::Word)
}

#[test]
fn bt_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EDI)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 231, 66], OperandSize::Dword)
}

#[test]
fn bt_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 36, 193, 73], OperandSize::Dword)
}

#[test]
fn bt_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EDI)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 231, 21], OperandSize::Qword)
}

#[test]
fn bt_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 33, 99], OperandSize::Qword)
}

#[test]
fn bt_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(RBP)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 229, 120], OperandSize::Qword)
}

#[test]
fn bt_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 277837812, Some(OperandSize::Qword), None)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 164, 86, 244, 119, 143, 16, 30], OperandSize::Qword)
}

