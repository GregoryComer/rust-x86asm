use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlvw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 139, 16, 215], OperandSize::Dword)
}

#[test]
fn vpsrlvw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1534757311, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 137, 16, 156, 187, 191, 137, 122, 91], OperandSize::Dword)
}

#[test]
fn vpsrlvw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 181, 130, 16, 255], OperandSize::Qword)
}

#[test]
fn vpsrlvw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RBX, 1276147186, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 237, 141, 16, 139, 242, 117, 16, 76], OperandSize::Qword)
}

#[test]
fn vpsrlvw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 171, 16, 234], OperandSize::Dword)
}

#[test]
fn vpsrlvw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 1682829661, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 171, 16, 169, 93, 241, 77, 100], OperandSize::Dword)
}

#[test]
fn vpsrlvw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 245, 171, 16, 205], OperandSize::Qword)
}

#[test]
fn vpsrlvw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RBX, 1398599224, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 181, 163, 16, 163, 56, 238, 92, 83], OperandSize::Qword)
}

#[test]
fn vpsrlvw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 204, 16, 221], OperandSize::Dword)
}

#[test]
fn vpsrlvw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 201, 16, 20, 87], OperandSize::Dword)
}

#[test]
fn vpsrlvw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 189, 199, 16, 231], OperandSize::Qword)
}

#[test]
fn vpsrlvw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 237, 205, 16, 44, 240], OperandSize::Qword)
}

