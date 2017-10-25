use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vscalefss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 254, 45, 228], OperandSize::Dword)
}

fn vscalefss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 140, 45, 43], OperandSize::Dword)
}

fn vscalefss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 85, 210, 45, 207], OperandSize::Qword)
}

fn vscalefss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 117, 142, 45, 40], OperandSize::Qword)
}

