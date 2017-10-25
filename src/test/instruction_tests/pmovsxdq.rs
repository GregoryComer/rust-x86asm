use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovsxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 249], OperandSize::Dword)
}

fn pmovsxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 25], OperandSize::Dword)
}

fn pmovsxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 200], OperandSize::Qword)
}

fn pmovsxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 34], OperandSize::Qword)
}

