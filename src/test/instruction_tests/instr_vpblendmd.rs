use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 139, 100, 236], OperandSize::Dword)
}

#[test]
fn vpblendmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 596128529, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 100, 168, 17, 51, 136, 35], OperandSize::Dword)
}

#[test]
fn vpblendmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 642550523, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 156, 100, 44, 253, 251, 138, 76, 38], OperandSize::Dword)
}

#[test]
fn vpblendmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 29, 143, 100, 208], OperandSize::Qword)
}

#[test]
fn vpblendmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 45, 130, 100, 14], OperandSize::Qword)
}

#[test]
fn vpblendmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 879335622, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 21, 158, 100, 148, 73, 198, 152, 105, 52], OperandSize::Qword)
}

#[test]
fn vpblendmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 100, 209], OperandSize::Dword)
}

#[test]
fn vpblendmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ESI, 1211180807, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 172, 100, 166, 7, 39, 49, 72], OperandSize::Dword)
}

#[test]
fn vpblendmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDI, 1293656355, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 188, 100, 167, 35, 161, 27, 77], OperandSize::Dword)
}

#[test]
fn vpblendmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 165, 100, 244], OperandSize::Qword)
}

#[test]
fn vpblendmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1155305210, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 45, 163, 100, 52, 125, 250, 142, 220, 68], OperandSize::Qword)
}

#[test]
fn vpblendmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 109, 185, 100, 35], OperandSize::Qword)
}

#[test]
fn vpblendmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 203, 100, 205], OperandSize::Dword)
}

#[test]
fn vpblendmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 1449515387, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 204, 100, 132, 210, 123, 217, 101, 86], OperandSize::Dword)
}

#[test]
fn vpblendmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1574802433, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 223, 100, 28, 221, 1, 148, 221, 93], OperandSize::Dword)
}

#[test]
fn vpblendmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 37, 194, 100, 254], OperandSize::Qword)
}

#[test]
fn vpblendmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 77, 196, 100, 44, 75], OperandSize::Qword)
}

#[test]
fn vpblendmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(RSI, 1075716636, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 101, 220, 100, 190, 28, 34, 30, 64], OperandSize::Qword)
}

