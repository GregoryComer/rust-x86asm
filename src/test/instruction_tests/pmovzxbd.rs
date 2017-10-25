use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovzxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 226], OperandSize::Dword)
}

fn pmovzxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 553099939, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 140, 83, 163, 162, 247, 32], OperandSize::Dword)
}

fn pmovzxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 226], OperandSize::Qword)
}

fn pmovzxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 273617822, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 172, 183, 158, 19, 79, 16], OperandSize::Qword)
}

