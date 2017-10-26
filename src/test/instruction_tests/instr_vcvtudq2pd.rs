use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtudq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 122, 192], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1930790699, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 139, 122, 36, 181, 43, 135, 21, 115], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 126, 139, 122, 218], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1518103564, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 126, 141, 122, 52, 213, 12, 108, 124, 90], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 122, 252], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1415757996, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 122, 36, 205, 172, 192, 98, 84], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 126, 171, 122, 244], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(RBX, 1103537311, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 173, 122, 139, 159, 164, 198, 65], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 122, 210], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1402134683, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 122, 28, 245, 155, 224, 146, 83], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 126, 205, 122, 250], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectDisplaced(RSI, 1286016286, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 126, 207, 122, 174, 30, 13, 167, 76], OperandSize::Qword)
}

