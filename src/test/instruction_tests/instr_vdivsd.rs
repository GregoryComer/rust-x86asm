use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 94, 216], OperandSize::Dword)
}

#[test]
fn vdivsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 733165336, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 94, 60, 77, 24, 55, 179, 43], OperandSize::Dword)
}

#[test]
fn vdivsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 94, 250], OperandSize::Qword)
}

#[test]
fn vdivsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1481626879, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 94, 132, 130, 255, 212, 79, 88], OperandSize::Qword)
}

#[test]
fn vdivsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 247, 251, 94, 207], OperandSize::Dword)
}

#[test]
fn vdivsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 223, 138, 94, 25], OperandSize::Dword)
}

#[test]
fn vdivsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 231, 180, 94, 216], OperandSize::Qword)
}

#[test]
fn vdivsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 247, 143, 94, 34], OperandSize::Qword)
}

