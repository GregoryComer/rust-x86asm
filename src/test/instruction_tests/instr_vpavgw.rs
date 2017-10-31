use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpavgw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 227, 216], OperandSize::Dword)
}

#[test]
fn vpavgw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 227, 28, 90], OperandSize::Dword)
}

#[test]
fn vpavgw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 227, 240], OperandSize::Qword)
}

#[test]
fn vpavgw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 590907135, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 227, 188, 203, 255, 134, 56, 35], OperandSize::Qword)
}

#[test]
fn vpavgw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 227, 194], OperandSize::Dword)
}

#[test]
fn vpavgw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 227, 12, 67], OperandSize::Dword)
}

#[test]
fn vpavgw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 227, 254], OperandSize::Qword)
}

#[test]
fn vpavgw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1104971393, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 227, 164, 120, 129, 134, 220, 65], OperandSize::Qword)
}

#[test]
fn vpavgw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 139, 227, 232], OperandSize::Dword)
}

#[test]
fn vpavgw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 358038822, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 140, 227, 12, 245, 38, 61, 87, 21], OperandSize::Dword)
}

#[test]
fn vpavgw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 61, 142, 227, 236], OperandSize::Qword)
}

#[test]
fn vpavgw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 312283505, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 61, 135, 227, 4, 77, 113, 17, 157, 18], OperandSize::Qword)
}

#[test]
fn vpavgw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 169, 227, 252], OperandSize::Dword)
}

#[test]
fn vpavgw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 172, 227, 52, 128], OperandSize::Dword)
}

#[test]
fn vpavgw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 21, 172, 227, 230], OperandSize::Qword)
}

#[test]
fn vpavgw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 21, 166, 227, 28, 82], OperandSize::Qword)
}

#[test]
fn vpavgw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 204, 227, 228], OperandSize::Dword)
}

#[test]
fn vpavgw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 207, 227, 4, 128], OperandSize::Dword)
}

#[test]
fn vpavgw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 77, 202, 227, 222], OperandSize::Qword)
}

#[test]
fn vpavgw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGW, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 401311194, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 117, 196, 227, 188, 89, 218, 133, 235, 23], OperandSize::Qword)
}

