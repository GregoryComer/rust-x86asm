use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpavgw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 227, 206], OperandSize::Dword)
}

#[test]
fn vpavgw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 821529206, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 227, 184, 118, 138, 247, 48], OperandSize::Dword)
}

#[test]
fn vpavgw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 227, 212], OperandSize::Qword)
}

#[test]
fn vpavgw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1814893921, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 227, 12, 157, 97, 21, 45, 108], OperandSize::Qword)
}

#[test]
fn vpavgw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 227, 224], OperandSize::Dword)
}

#[test]
fn vpavgw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1182858281, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 227, 180, 147, 41, 252, 128, 70], OperandSize::Dword)
}

#[test]
fn vpavgw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 227, 193], OperandSize::Qword)
}

#[test]
fn vpavgw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RAX, 1843828696, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 227, 128, 216, 151, 230, 109], OperandSize::Qword)
}

#[test]
fn vpavgw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 141, 227, 251], OperandSize::Dword)
}

#[test]
fn vpavgw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 137, 227, 44, 190], OperandSize::Dword)
}

#[test]
fn vpavgw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 29, 141, 227, 242], OperandSize::Qword)
}

#[test]
fn vpavgw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1981754479, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 53, 138, 227, 20, 213, 111, 44, 31, 118], OperandSize::Qword)
}

#[test]
fn vpavgw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 175, 227, 199], OperandSize::Dword)
}

#[test]
fn vpavgw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 253703776, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 169, 227, 12, 133, 96, 54, 31, 15], OperandSize::Dword)
}

#[test]
fn vpavgw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 101, 167, 227, 195], OperandSize::Qword)
}

#[test]
fn vpavgw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 5, 162, 227, 3], OperandSize::Qword)
}

#[test]
fn vpavgw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 201, 227, 205], OperandSize::Dword)
}

#[test]
fn vpavgw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EBX, 1986335733, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 205, 227, 131, 245, 19, 101, 118], OperandSize::Dword)
}

#[test]
fn vpavgw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 21, 204, 227, 223], OperandSize::Qword)
}

#[test]
fn vpavgw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM23)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 69, 196, 227, 10], OperandSize::Qword)
}

