use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movshdup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 250], OperandSize::Dword)
}

fn movshdup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1042872084, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 148, 150, 20, 247, 40, 62], OperandSize::Dword)
}

fn movshdup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 230], OperandSize::Qword)
}

fn movshdup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RCX, 1697281642, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 129, 106, 118, 42, 101], OperandSize::Qword)
}

