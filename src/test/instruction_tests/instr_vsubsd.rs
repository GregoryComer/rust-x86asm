use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 92, 219], OperandSize::Dword)
}

#[test]
fn vsubsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 1323778319, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 92, 132, 142, 15, 65, 231, 78], OperandSize::Dword)
}

#[test]
fn vsubsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 92, 212], OperandSize::Qword)
}

#[test]
fn vsubsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RCX, 1876202076, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 92, 185, 92, 146, 212, 111], OperandSize::Qword)
}

#[test]
fn vsubsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 207, 221, 92, 246], OperandSize::Dword)
}

#[test]
fn vsubsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1324529119, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 138, 92, 60, 197, 223, 181, 242, 78], OperandSize::Dword)
}

#[test]
fn vsubsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 135, 156, 92, 242], OperandSize::Qword)
}

#[test]
fn vsubsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 343074211, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 191, 139, 92, 4, 181, 163, 229, 114, 20], OperandSize::Qword)
}

