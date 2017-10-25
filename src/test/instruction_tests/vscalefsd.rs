use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vscalefsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 254, 45, 242], OperandSize::Dword)
}

fn vscalefsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 139, 45, 9], OperandSize::Dword)
}

fn vscalefsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 197, 214, 45, 252], OperandSize::Qword)
}

fn vscalefsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectDisplaced(RAX, 901452683, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 157, 143, 45, 144, 139, 19, 187, 53], OperandSize::Qword)
}

