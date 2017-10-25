use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtss2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 232], OperandSize::Dword)
}

fn cvtss2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 22132724, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 60, 157, 244, 183, 81, 1], OperandSize::Dword)
}

fn cvtss2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 241], OperandSize::Qword)
}

fn cvtss2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 20, 243], OperandSize::Qword)
}

