use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpblendmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 138, 100, 198], OperandSize::Dword)
}

fn vpblendmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 140, 100, 14], OperandSize::Dword)
}

fn vpblendmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 159, 100, 4, 246], OperandSize::Dword)
}

fn vpblendmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 69, 139, 100, 239], OperandSize::Qword)
}

fn vpblendmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1483907111, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 53, 135, 100, 20, 253, 39, 160, 114, 88], OperandSize::Qword)
}

fn vpblendmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDX, 1437419980, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 159, 100, 146, 204, 73, 173, 85], OperandSize::Qword)
}

fn vpblendmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 171, 100, 228], OperandSize::Dword)
}

fn vpblendmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 630843225, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 175, 100, 159, 89, 231, 153, 37], OperandSize::Dword)
}

fn vpblendmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1051952291, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 191, 100, 188, 153, 163, 132, 179, 62], OperandSize::Dword)
}

fn vpblendmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 165, 100, 250], OperandSize::Qword)
}

fn vpblendmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM29)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 21, 161, 100, 62], OperandSize::Qword)
}

fn vpblendmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 53, 190, 100, 44, 126], OperandSize::Qword)
}

fn vpblendmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 100, 244], OperandSize::Dword)
}

fn vpblendmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 100, 18], OperandSize::Dword)
}

fn vpblendmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1818856206, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 100, 4, 69, 14, 139, 105, 108], OperandSize::Dword)
}

fn vpblendmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 21, 202, 100, 248], OperandSize::Qword)
}

fn vpblendmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 39317517, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 29, 194, 100, 164, 210, 13, 240, 87, 2], OperandSize::Qword)
}

fn vpblendmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM28)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 29, 214, 100, 63], OperandSize::Qword)
}

