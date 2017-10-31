use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpavgw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 227, 202], OperandSize::Dword)
}

#[test]
fn vpavgw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 1825231390, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 227, 132, 120, 30, 210, 202, 108], OperandSize::Dword)
}

#[test]
fn vpavgw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 227, 203], OperandSize::Qword)
}

#[test]
fn vpavgw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1382624171, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 227, 148, 82, 171, 43, 105, 82], OperandSize::Qword)
}

#[test]
fn vpavgw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 227, 198], OperandSize::Dword)
}

#[test]
fn vpavgw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 227, 44, 207], OperandSize::Dword)
}

#[test]
fn vpavgw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 227, 201], OperandSize::Qword)
}

#[test]
fn vpavgw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RBX, 2117644854, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 227, 179, 54, 178, 56, 126], OperandSize::Qword)
}

#[test]
fn vpavgw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 227, 239], OperandSize::Dword)
}

#[test]
fn vpavgw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1526155249, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 141, 227, 4, 213, 241, 71, 247, 90], OperandSize::Dword)
}

#[test]
fn vpavgw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 69, 143, 227, 223], OperandSize::Qword)
}

#[test]
fn vpavgw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 13, 132, 227, 36, 179], OperandSize::Qword)
}

#[test]
fn vpavgw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 227, 234], OperandSize::Dword)
}

#[test]
fn vpavgw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 1137167369, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 173, 227, 172, 194, 9, 204, 199, 67], OperandSize::Dword)
}

#[test]
fn vpavgw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 5, 163, 227, 210], OperandSize::Qword)
}

#[test]
fn vpavgw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectDisplaced(RCX, 1084811640, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 162, 227, 169, 120, 233, 168, 64], OperandSize::Qword)
}

#[test]
fn vpavgw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 202, 227, 245], OperandSize::Dword)
}

#[test]
fn vpavgw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 205, 227, 44, 114], OperandSize::Dword)
}

#[test]
fn vpavgw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 5, 194, 227, 210], OperandSize::Qword)
}

#[test]
fn vpavgw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(RAX, 563196211, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 109, 201, 227, 176, 51, 177, 145, 33], OperandSize::Qword)
}

