use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovsxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 210], OperandSize::Dword)
}

fn pmovsxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1808634562, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 12, 157, 194, 146, 205, 107], OperandSize::Dword)
}

fn pmovsxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 209], OperandSize::Qword)
}

fn pmovsxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 382799384, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 36, 117, 24, 14, 209, 22], OperandSize::Qword)
}

