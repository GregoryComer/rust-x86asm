use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 94, 198], OperandSize::Dword)
}

#[test]
fn vdivss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 316982267, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 94, 180, 254, 251, 195, 228, 18], OperandSize::Dword)
}

#[test]
fn vdivss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 94, 243], OperandSize::Qword)
}

#[test]
fn vdivss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RSI, 1623275847, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 94, 150, 71, 57, 193, 96], OperandSize::Qword)
}

#[test]
fn vdivss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 190, 94, 249], OperandSize::Dword)
}

#[test]
fn vdivss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 94, 143, 94, 57], OperandSize::Dword)
}

#[test]
fn vdivss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 126, 211, 94, 215], OperandSize::Qword)
}

#[test]
fn vdivss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 110, 134, 94, 7], OperandSize::Qword)
}

