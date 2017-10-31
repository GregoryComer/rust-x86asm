use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 153, 45, 223], OperandSize::Dword)
}

#[test]
fn vscalefsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 45, 12, 136], OperandSize::Dword)
}

#[test]
fn vscalefsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 141, 177, 45, 244], OperandSize::Qword)
}

#[test]
fn vscalefsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 253, 133, 45, 20, 223], OperandSize::Qword)
}

