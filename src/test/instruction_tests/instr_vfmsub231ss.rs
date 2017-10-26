use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 187, 250], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 1768728292, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 187, 164, 209, 228, 166, 108, 105], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 187, 235], OperandSize::Qword)
}

#[test]
fn vfmsub231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 187, 33], OperandSize::Qword)
}

#[test]
fn vfmsub231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 153, 187, 221], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 140, 187, 60, 254], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 45, 155, 187, 252], OperandSize::Qword)
}

#[test]
fn vfmsub231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 29, 132, 187, 36, 187], OperandSize::Qword)
}

