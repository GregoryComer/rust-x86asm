use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn popcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 246], OperandSize::Word)
}

#[test]
fn popcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(DI, 24, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 85, 24], OperandSize::Word)
}

#[test]
fn popcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 205], OperandSize::Dword)
}

#[test]
fn popcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(SI)), operand2: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 54], OperandSize::Dword)
}

#[test]
fn popcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 241], OperandSize::Qword)
}

#[test]
fn popcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(DX)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 18], OperandSize::Qword)
}

#[test]
fn popcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 215], OperandSize::Word)
}

#[test]
fn popcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(DI, 8, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 93, 8], OperandSize::Word)
}

#[test]
fn popcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 233], OperandSize::Dword)
}

#[test]
fn popcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 44, 176], OperandSize::Dword)
}

#[test]
fn popcnt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 206], OperandSize::Qword)
}

#[test]
fn popcnt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EBP)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 46], OperandSize::Qword)
}

#[test]
fn popcnt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 184, 223], OperandSize::Qword)
}

#[test]
fn popcnt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(RSI)), operand2: Some(IndirectDisplaced(RSI, 717240050, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 184, 182, 242, 54, 192, 42], OperandSize::Qword)
}

