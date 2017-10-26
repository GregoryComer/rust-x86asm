use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 142, 114, 196, 81], OperandSize::Dword)
}

#[test]
fn vprord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 142, 114, 4, 81, 19], OperandSize::Dword)
}

#[test]
fn vprord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 85, 159, 114, 7, 2], OperandSize::Dword)
}

#[test]
fn vprord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM9)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 101, 141, 114, 193, 24], OperandSize::Qword)
}

#[test]
fn vprord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM25)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 53, 131, 114, 2, 5], OperandSize::Qword)
}

#[test]
fn vprord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 156, 114, 4, 89, 38], OperandSize::Qword)
}

#[test]
fn vprord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 114, 192, 26], OperandSize::Dword)
}

#[test]
fn vprord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1837547730, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 172, 114, 132, 67, 210, 192, 134, 109, 35], OperandSize::Dword)
}

#[test]
fn vprord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 69, 190, 114, 7, 122], OperandSize::Dword)
}

#[test]
fn vprord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 37, 165, 114, 199, 67], OperandSize::Qword)
}

#[test]
fn vprord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 2125400395, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 165, 114, 4, 253, 75, 9, 175, 126, 23], OperandSize::Qword)
}

#[test]
fn vprord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 77, 191, 114, 4, 210, 121], OperandSize::Qword)
}

#[test]
fn vprord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 203, 114, 197, 5], OperandSize::Dword)
}

#[test]
fn vprord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 406694341, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 114, 4, 117, 197, 169, 61, 24, 79], OperandSize::Dword)
}

#[test]
fn vprord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 217, 114, 4, 65, 71], OperandSize::Dword)
}

#[test]
fn vprord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 45, 198, 114, 199, 106], OperandSize::Qword)
}

#[test]
fn vprord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 311329178, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 203, 114, 4, 189, 154, 129, 142, 18, 115], OperandSize::Qword)
}

#[test]
fn vprord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectDisplaced(RBX, 795128512, Some(OperandSize::Dword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 117, 214, 114, 131, 192, 178, 100, 47, 91], OperandSize::Qword)
}

