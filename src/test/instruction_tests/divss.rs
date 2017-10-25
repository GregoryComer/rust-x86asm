use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn divss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 239], OperandSize::Dword)
}

fn divss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ESI, 545469592, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 182, 152, 52, 131, 32], OperandSize::Dword)
}

fn divss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 242], OperandSize::Qword)
}

fn divss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 167330601, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 60, 149, 41, 67, 249, 9], OperandSize::Qword)
}

