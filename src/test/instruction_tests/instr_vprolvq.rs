use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprolvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 139, 21, 244], OperandSize::Dword)
}

#[test]
fn vprolvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 37026103, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 21, 180, 143, 55, 249, 52, 2], OperandSize::Dword)
}

#[test]
fn vprolvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 1366565386, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 155, 21, 187, 10, 34, 116, 81], OperandSize::Dword)
}

#[test]
fn vprolvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 173, 141, 21, 211], OperandSize::Qword)
}

#[test]
fn vprolvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1661857326, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 237, 141, 21, 156, 118, 46, 238, 13, 99], OperandSize::Qword)
}

#[test]
fn vprolvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 157, 153, 21, 2], OperandSize::Qword)
}

#[test]
fn vprolvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 169, 21, 236], OperandSize::Dword)
}

#[test]
fn vprolvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EAX, 369870858, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 170, 21, 136, 10, 200, 11, 22], OperandSize::Dword)
}

#[test]
fn vprolvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 187, 21, 52, 199], OperandSize::Dword)
}

#[test]
fn vprolvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 157, 172, 21, 219], OperandSize::Qword)
}

#[test]
fn vprolvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1355043801, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 157, 163, 21, 12, 69, 217, 83, 196, 80], OperandSize::Qword)
}

#[test]
fn vprolvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 516309811, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 229, 185, 21, 36, 141, 51, 67, 198, 30], OperandSize::Qword)
}

#[test]
fn vprolvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 206, 21, 201], OperandSize::Dword)
}

#[test]
fn vprolvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 354473001, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 21, 148, 120, 41, 212, 32, 21], OperandSize::Dword)
}

#[test]
fn vprolvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDI, 1890967536, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 217, 21, 175, 240, 223, 181, 112], OperandSize::Dword)
}

#[test]
fn vprolvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 173, 193, 21, 249], OperandSize::Qword)
}

#[test]
fn vprolvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM22)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 205, 193, 21, 50], OperandSize::Qword)
}

#[test]
fn vprolvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectDisplaced(RDI, 463652428, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 141, 210, 21, 159, 76, 198, 162, 27], OperandSize::Qword)
}

