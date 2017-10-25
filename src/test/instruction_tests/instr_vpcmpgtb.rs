use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 100, 246], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 682414884, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 100, 137, 36, 211, 172, 40], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 100, 196], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDX, 542853453, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 100, 178, 77, 73, 91, 32], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 100, 199], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 1248351320, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 100, 164, 126, 88, 84, 104, 74], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 100, 236], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RBX, 1569582787, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 100, 179, 195, 238, 141, 93], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 10, 100, 237], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 736265292, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 9, 100, 36, 125, 76, 132, 226, 43], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 69, 15, 100, 202], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1050214521, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 3, 100, 44, 221, 121, 0, 153, 62], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 47, 100, 223], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 496385117, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 46, 100, 28, 213, 93, 60, 150, 29], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 5, 39, 100, 251], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM24)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 61, 39, 100, 49], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 73, 100, 251], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDX, 1572773301, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 73, 100, 178, 181, 157, 190, 93], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 37, 78, 100, 215], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 522813802, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 53, 69, 100, 148, 139, 106, 129, 41, 31], OperandSize::Qword)
}

