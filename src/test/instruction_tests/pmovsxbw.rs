use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovsxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 213], OperandSize::Dword)
}

fn pmovsxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 24], OperandSize::Dword)
}

fn pmovsxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 204], OperandSize::Qword)
}

fn pmovsxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1701007431, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 164, 70, 71, 80, 99, 101], OperandSize::Qword)
}

