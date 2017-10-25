use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtss2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 90, 246], OperandSize::Dword)
}

fn vcvtss2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 90, 44, 191], OperandSize::Dword)
}

fn vcvtss2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 90, 235], OperandSize::Qword)
}

fn vcvtss2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 90, 9], OperandSize::Qword)
}

fn vcvtss2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 70, 156, 90, 248], OperandSize::Dword)
}

fn vcvtss2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 875956630, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 118, 142, 90, 20, 157, 150, 9, 54, 52], OperandSize::Dword)
}

fn vcvtss2sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 118, 151, 90, 251], OperandSize::Qword)
}

fn vcvtss2sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 2081233890, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 46, 141, 90, 44, 157, 226, 27, 13, 124], OperandSize::Qword)
}

