use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vaesdeclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 223, 200], OperandSize::Dword)
}

fn vaesdeclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ECX, 344209961, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 223, 169, 41, 58, 132, 20], OperandSize::Dword)
}

fn vaesdeclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 223, 247], OperandSize::Qword)
}

fn vaesdeclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 555324254, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 223, 28, 181, 94, 147, 25, 33], OperandSize::Qword)
}

