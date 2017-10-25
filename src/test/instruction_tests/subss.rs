use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn subss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 246], OperandSize::Dword)
}

fn subss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 6], OperandSize::Dword)
}

fn subss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 224], OperandSize::Qword)
}

fn subss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 1789096616, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 180, 136, 168, 114, 163, 106], OperandSize::Qword)
}

