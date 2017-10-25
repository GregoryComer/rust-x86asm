use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pcmpgtq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 209], OperandSize::Dword)
}

fn pcmpgtq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EAX, 2035729020, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 176, 124, 194, 86, 121], OperandSize::Dword)
}

fn pcmpgtq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 245], OperandSize::Qword)
}

fn pcmpgtq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RCX, 933422945, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 129, 97, 231, 162, 55], OperandSize::Qword)
}

