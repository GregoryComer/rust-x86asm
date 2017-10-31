use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpconflictq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 196, 206], OperandSize::Dword)
}

#[test]
fn vpconflictq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 196, 0], OperandSize::Dword)
}

#[test]
fn vpconflictq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 351916255, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 196, 148, 138, 223, 208, 249, 20], OperandSize::Dword)
}

#[test]
fn vpconflictq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 253, 142, 196, 192], OperandSize::Qword)
}

#[test]
fn vpconflictq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM20)), operand2: Some(IndirectDisplaced(RDI, 680314502, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 253, 143, 196, 167, 134, 198, 140, 40], OperandSize::Qword)
}

#[test]
fn vpconflictq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1226531600, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 253, 155, 196, 156, 182, 16, 99, 27, 73], OperandSize::Qword)
}

#[test]
fn vpconflictq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 196, 225], OperandSize::Dword)
}

#[test]
fn vpconflictq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1330533715, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 196, 36, 181, 83, 85, 78, 79], OperandSize::Dword)
}

#[test]
fn vpconflictq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 188, 196, 20, 151], OperandSize::Dword)
}

#[test]
fn vpconflictq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 253, 172, 196, 252], OperandSize::Qword)
}

#[test]
fn vpconflictq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 253, 169, 196, 4, 131], OperandSize::Qword)
}

#[test]
fn vpconflictq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1806444664, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 253, 189, 196, 164, 147, 120, 40, 172, 107], OperandSize::Qword)
}

#[test]
fn vpconflictq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 196, 233], OperandSize::Dword)
}

#[test]
fn vpconflictq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1941989452, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 196, 172, 254, 76, 104, 192, 115], OperandSize::Dword)
}

#[test]
fn vpconflictq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 196, 32], OperandSize::Dword)
}

#[test]
fn vpconflictq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 253, 203, 196, 202], OperandSize::Qword)
}

#[test]
fn vpconflictq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 196, 49], OperandSize::Qword)
}

#[test]
fn vpconflictq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectDisplaced(RAX, 1101023124, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 253, 218, 196, 128, 148, 71, 160, 65], OperandSize::Qword)
}

