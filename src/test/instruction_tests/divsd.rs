use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn divsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 236], OperandSize::Dword)
}

fn divsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 1114058905, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 140, 142, 153, 48, 103, 66], OperandSize::Dword)
}

fn divsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 251], OperandSize::Qword)
}

fn divsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 1], OperandSize::Qword)
}

