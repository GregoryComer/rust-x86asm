use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ucomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 235], OperandSize::Dword)
}

#[test]
fn ucomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1486619889, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 20, 205, 241, 4, 156, 88], OperandSize::Dword)
}

#[test]
fn ucomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 197], OperandSize::Qword)
}

#[test]
fn ucomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 0], OperandSize::Qword)
}

