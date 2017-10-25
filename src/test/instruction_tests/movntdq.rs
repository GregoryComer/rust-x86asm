use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movntdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQ, operand1: Some(IndirectDisplaced(EDX, 1530437504, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 231, 170, 128, 159, 56, 91], OperandSize::Dword)
}

fn movntdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQ, operand1: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 231, 49], OperandSize::Qword)
}

