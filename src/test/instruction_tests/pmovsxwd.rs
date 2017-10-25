use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovsxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 241], OperandSize::Dword)
}

fn pmovsxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 28, 249], OperandSize::Dword)
}

fn pmovsxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 234], OperandSize::Qword)
}

fn pmovsxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 48], OperandSize::Qword)
}

