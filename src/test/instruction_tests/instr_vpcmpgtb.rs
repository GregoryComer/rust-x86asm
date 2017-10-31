use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 100, 243], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 100, 62], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 100, 219], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RBX, 125214763, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 100, 179, 43, 160, 118, 7], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 100, 204], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 903752106, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 100, 188, 194, 170, 41, 222, 53], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 100, 225], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 1928529614, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 100, 132, 136, 206, 6, 243, 114], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 9, 100, 252], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 365114967, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 10, 100, 44, 245, 87, 54, 195, 21], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 109, 4, 100, 224], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectDisplaced(RAX, 1656607684, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 5, 100, 184, 196, 211, 189, 98], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 46, 100, 219], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1372695290, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 47, 100, 140, 214, 250, 170, 209, 81], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 85, 44, 100, 237], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM15)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 5, 44, 100, 63], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 78, 100, 230], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 79, 100, 36, 73], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 61, 75, 100, 202], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1190461983, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 77, 100, 156, 131, 31, 2, 245, 70], OperandSize::Qword)
}

