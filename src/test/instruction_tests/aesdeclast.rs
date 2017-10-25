use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn aesdeclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 197], OperandSize::Dword)
}

fn aesdeclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 20, 94], OperandSize::Dword)
}

fn aesdeclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 198], OperandSize::Qword)
}

fn aesdeclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RSI, 569543313, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 142, 145, 138, 242, 33], OperandSize::Qword)
}

