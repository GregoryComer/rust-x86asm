use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 116, 252], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDI, 1074219882, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 116, 159, 106, 75, 7, 64], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 116, 214], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 1469207108, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 116, 180, 159, 68, 82, 146, 87], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 116, 233], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1175804749, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 116, 132, 146, 77, 91, 21, 70], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 116, 242], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 116, 60, 150], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 11, 116, 253], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 2094222551, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 14, 116, 28, 93, 215, 76, 211, 124], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 53, 13, 116, 218], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 2115397414, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 1, 116, 36, 125, 38, 103, 22, 126], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 45, 116, 222], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 42, 116, 12, 214], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 13, 47, 116, 240], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectDisplaced(RBX, 720011766, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 45, 36, 116, 163, 246, 129, 234, 42], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 78, 116, 246], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 87880443, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 76, 116, 140, 211, 251, 242, 60, 5], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 61, 70, 116, 247], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 71, 116, 52, 247], OperandSize::Qword)
}

