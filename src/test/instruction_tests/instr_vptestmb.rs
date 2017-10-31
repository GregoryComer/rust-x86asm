use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 14, 38, 223], OperandSize::Dword)
}

#[test]
fn vptestmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 12, 38, 12, 249], OperandSize::Dword)
}

#[test]
fn vptestmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 45, 12, 38, 234], OperandSize::Qword)
}

#[test]
fn vptestmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1766217770, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 5, 38, 12, 69, 42, 88, 70, 105], OperandSize::Qword)
}

#[test]
fn vptestmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 45, 38, 207], OperandSize::Dword)
}

#[test]
fn vptestmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 44, 38, 12, 91], OperandSize::Dword)
}

#[test]
fn vptestmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 77, 45, 38, 222], OperandSize::Qword)
}

#[test]
fn vptestmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectDisplaced(RDI, 789335474, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 29, 43, 38, 167, 178, 77, 12, 47], OperandSize::Qword)
}

#[test]
fn vptestmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 79, 38, 226], OperandSize::Dword)
}

#[test]
fn vptestmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 79, 38, 52, 155], OperandSize::Dword)
}

#[test]
fn vptestmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 69, 70, 38, 219], OperandSize::Qword)
}

#[test]
fn vptestmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 77, 38, 52, 248], OperandSize::Qword)
}

