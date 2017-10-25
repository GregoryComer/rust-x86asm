use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 190, 45, 209], OperandSize::Dword)
}

#[test]
fn vscalefss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 45, 0], OperandSize::Dword)
}

#[test]
fn vscalefss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 53, 157, 45, 228], OperandSize::Qword)
}

#[test]
fn vscalefss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1619830336, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 61, 142, 45, 180, 250, 64, 166, 140, 96], OperandSize::Qword)
}

