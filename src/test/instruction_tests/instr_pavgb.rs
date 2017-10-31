use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pavgb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 210], OperandSize::Dword)
}

#[test]
fn pavgb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM5)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 42], OperandSize::Dword)
}

#[test]
fn pavgb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 234], OperandSize::Qword)
}

#[test]
fn pavgb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 560844359, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 140, 121, 71, 206, 109, 33], OperandSize::Qword)
}

#[test]
fn pavgb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 197], OperandSize::Dword)
}

#[test]
fn pavgb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 1593432830, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 156, 89, 254, 218, 249, 94], OperandSize::Dword)
}

#[test]
fn pavgb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 192], OperandSize::Qword)
}

#[test]
fn pavgb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 312718139, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 52, 85, 59, 179, 163, 18], OperandSize::Qword)
}

