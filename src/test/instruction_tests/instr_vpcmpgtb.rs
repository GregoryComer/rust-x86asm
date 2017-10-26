use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 100, 235], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 100, 11], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 100, 197], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 100, 44, 135], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 100, 199], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 1414537322, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 100, 137, 106, 32, 80, 84], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 100, 196], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RDX, 546739994, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 100, 138, 26, 151, 150, 32], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 15, 100, 224], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 11, 100, 30], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 13, 7, 100, 207], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 2055991629, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 7, 100, 20, 117, 77, 241, 139, 122], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 41, 100, 237], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 644210677, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 41, 100, 36, 133, 245, 223, 101, 38], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 29, 43, 100, 231], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RSI, 1030062251, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 21, 41, 100, 166, 171, 128, 101, 61], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 73, 100, 203], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1805221685, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 74, 100, 188, 131, 53, 127, 153, 107], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 5, 66, 100, 228], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectDisplaced(RDI, 608390375, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 29, 79, 100, 159, 231, 76, 67, 36], OperandSize::Qword)
}

