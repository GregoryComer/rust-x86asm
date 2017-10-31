use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 10, 38, 217], OperandSize::Dword)
}

#[test]
fn vptestmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 427244617, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 15, 38, 187, 73, 60, 119, 25], OperandSize::Dword)
}

#[test]
fn vptestmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 221, 11, 38, 202], OperandSize::Qword)
}

#[test]
fn vptestmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 4, 38, 52, 135], OperandSize::Qword)
}

#[test]
fn vptestmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 45, 38, 226], OperandSize::Dword)
}

#[test]
fn vptestmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 47412936, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 43, 38, 172, 214, 200, 118, 211, 2], OperandSize::Dword)
}

#[test]
fn vptestmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 221, 47, 38, 233], OperandSize::Qword)
}

#[test]
fn vptestmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 181, 36, 38, 28, 219], OperandSize::Qword)
}

#[test]
fn vptestmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 74, 38, 240], OperandSize::Dword)
}

#[test]
fn vptestmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 353442478, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 79, 38, 36, 157, 174, 26, 17, 21], OperandSize::Dword)
}

#[test]
fn vptestmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 149, 79, 38, 234], OperandSize::Qword)
}

#[test]
fn vptestmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1917044996, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 149, 65, 38, 28, 125, 4, 201, 67, 114], OperandSize::Qword)
}

