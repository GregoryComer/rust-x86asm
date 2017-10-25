use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movq2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVQ2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 214, 192], OperandSize::Word)
}

fn movq2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVQ2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 214, 224], OperandSize::Dword)
}

fn movq2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVQ2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 214, 250], OperandSize::Qword)
}

