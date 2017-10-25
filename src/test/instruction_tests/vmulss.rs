use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmulss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 89, 220], OperandSize::Dword)
}

fn vmulss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 234613144, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 89, 148, 130, 152, 233, 251, 13], OperandSize::Dword)
}

fn vmulss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 89, 206], OperandSize::Qword)
}

fn vmulss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 89, 55], OperandSize::Qword)
}

fn vmulss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 110, 249, 89, 248], OperandSize::Dword)
}

fn vmulss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 70, 140, 89, 50], OperandSize::Dword)
}

fn vmulss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 14, 222, 89, 208], OperandSize::Qword)
}

fn vmulss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 78, 141, 89, 28, 208], OperandSize::Qword)
}

