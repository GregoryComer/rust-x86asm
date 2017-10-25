use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 11, 38, 201], OperandSize::Dword)
}

#[test]
fn vptestmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 9, 38, 52, 176], OperandSize::Dword)
}

#[test]
fn vptestmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 149, 15, 38, 205], OperandSize::Qword)
}

#[test]
fn vptestmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 250197136, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 189, 5, 38, 36, 117, 144, 180, 233, 14], OperandSize::Qword)
}

#[test]
fn vptestmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 47, 38, 210], OperandSize::Dword)
}

#[test]
fn vptestmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 134930632, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 43, 38, 140, 153, 200, 224, 10, 8], OperandSize::Dword)
}

#[test]
fn vptestmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 237, 44, 38, 202], OperandSize::Qword)
}

#[test]
fn vptestmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 133, 41, 38, 28, 147], OperandSize::Qword)
}

#[test]
fn vptestmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 77, 38, 251], OperandSize::Dword)
}

#[test]
fn vptestmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 73, 38, 14], OperandSize::Dword)
}

#[test]
fn vptestmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 133, 67, 38, 200], OperandSize::Qword)
}

#[test]
fn vptestmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectDisplaced(RDI, 1572969689, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 149, 78, 38, 191, 217, 156, 193, 93], OperandSize::Qword)
}

