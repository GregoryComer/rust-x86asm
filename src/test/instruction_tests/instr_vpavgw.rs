use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpavgw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 227, 246], OperandSize::Dword)
}

#[test]
fn vpavgw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 1574245385, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 227, 164, 74, 9, 20, 213, 93], OperandSize::Dword)
}

#[test]
fn vpavgw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 227, 221], OperandSize::Qword)
}

#[test]
fn vpavgw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 341895677, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 227, 156, 147, 253, 233, 96, 20], OperandSize::Qword)
}

#[test]
fn vpavgw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 227, 233], OperandSize::Dword)
}

#[test]
fn vpavgw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 227, 4, 194], OperandSize::Dword)
}

#[test]
fn vpavgw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 227, 248], OperandSize::Qword)
}

#[test]
fn vpavgw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1704573229, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 227, 36, 85, 45, 185, 153, 101], OperandSize::Qword)
}

#[test]
fn vpavgw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 227, 221], OperandSize::Dword)
}

#[test]
fn vpavgw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 523269262, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 140, 227, 60, 221, 142, 116, 48, 31], OperandSize::Dword)
}

#[test]
fn vpavgw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 13, 129, 227, 202], OperandSize::Qword)
}

#[test]
fn vpavgw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 13, 131, 227, 4, 67], OperandSize::Qword)
}

#[test]
fn vpavgw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 173, 227, 202], OperandSize::Dword)
}

#[test]
fn vpavgw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 505970452, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 227, 180, 192, 20, 127, 40, 30], OperandSize::Dword)
}

#[test]
fn vpavgw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 5, 174, 227, 237], OperandSize::Qword)
}

#[test]
fn vpavgw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1988439490, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 77, 162, 227, 12, 181, 194, 45, 133, 118], OperandSize::Qword)
}

#[test]
fn vpavgw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 202, 227, 251], OperandSize::Dword)
}

#[test]
fn vpavgw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 900824712, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 203, 227, 172, 115, 136, 126, 177, 53], OperandSize::Dword)
}

#[test]
fn vpavgw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 109, 199, 227, 225], OperandSize::Qword)
}

#[test]
fn vpavgw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RBX, 63933676, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 69, 204, 227, 131, 236, 140, 207, 3], OperandSize::Qword)
}

