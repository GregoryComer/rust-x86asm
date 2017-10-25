use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vgetexppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 66, 239], OperandSize::Dword)
}

fn vgetexppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 982965102, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 66, 188, 74, 110, 219, 150, 58], OperandSize::Dword)
}

fn vgetexppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EAX, 1985834318, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 158, 66, 184, 78, 109, 93, 118], OperandSize::Dword)
}

fn vgetexppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 253, 143, 66, 205], OperandSize::Qword)
}

fn vgetexppd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM22)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 253, 139, 66, 52, 75], OperandSize::Qword)
}

fn vgetexppd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 158, 66, 60, 184], OperandSize::Qword)
}

fn vgetexppd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 66, 225], OperandSize::Dword)
}

fn vgetexppd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 66, 4, 71], OperandSize::Dword)
}

fn vgetexppd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1825422631, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 187, 66, 148, 144, 39, 189, 205, 108], OperandSize::Dword)
}

fn vgetexppd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 253, 170, 66, 216], OperandSize::Qword)
}

fn vgetexppd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM17)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 253, 172, 66, 14], OperandSize::Qword)
}

fn vgetexppd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 572496127, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 189, 66, 60, 85, 255, 152, 31, 34], OperandSize::Qword)
}

fn vgetexppd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 158, 66, 238], OperandSize::Dword)
}

fn vgetexppd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1890372308, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 66, 172, 177, 212, 202, 172, 112], OperandSize::Dword)
}

fn vgetexppd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 882494181, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 220, 66, 164, 184, 229, 202, 153, 52], OperandSize::Dword)
}

fn vgetexppd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 253, 156, 66, 243], OperandSize::Qword)
}

fn vgetexppd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 426245535, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 253, 207, 66, 148, 152, 159, 253, 103, 25], OperandSize::Qword)
}

fn vgetexppd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectDisplaced(RAX, 1641363082, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 220, 66, 168, 138, 54, 213, 97], OperandSize::Qword)
}

