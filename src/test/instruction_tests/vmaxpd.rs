use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmaxpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 95, 221], OperandSize::Dword)
}

fn vmaxpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 95, 28, 120], OperandSize::Dword)
}

fn vmaxpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 95, 249], OperandSize::Qword)
}

fn vmaxpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RSI, 1532699335, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 95, 134, 199, 34, 91, 91], OperandSize::Qword)
}

fn vmaxpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 95, 219], OperandSize::Dword)
}

fn vmaxpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 95, 44, 122], OperandSize::Dword)
}

fn vmaxpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 95, 200], OperandSize::Qword)
}

fn vmaxpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RBX, 1185165202, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 95, 171, 146, 47, 164, 70], OperandSize::Qword)
}

fn vmaxpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 138, 95, 250], OperandSize::Dword)
}

fn vmaxpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ECX, 2077498307, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 95, 185, 195, 27, 212, 123], OperandSize::Dword)
}

fn vmaxpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 210975327, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 157, 95, 4, 245, 95, 58, 147, 12], OperandSize::Dword)
}

fn vmaxpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 253, 134, 95, 245], OperandSize::Qword)
}

fn vmaxpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 213, 140, 95, 12, 198], OperandSize::Qword)
}

fn vmaxpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDI, 695947767, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 159, 95, 167, 247, 81, 123, 41], OperandSize::Qword)
}

fn vmaxpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 172, 95, 192], OperandSize::Dword)
}

fn vmaxpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 174, 95, 40], OperandSize::Dword)
}

fn vmaxpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 186, 95, 54], OperandSize::Dword)
}

fn vmaxpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 165, 169, 95, 246], OperandSize::Qword)
}

fn vmaxpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 1245821549, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 133, 171, 95, 140, 183, 109, 186, 65, 74], OperandSize::Qword)
}

fn vmaxpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1858480538, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 205, 178, 95, 36, 157, 154, 41, 198, 110], OperandSize::Qword)
}

fn vmaxpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 156, 95, 243], OperandSize::Dword)
}

fn vmaxpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 207, 95, 12, 137], OperandSize::Dword)
}

fn vmaxpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 220, 95, 4, 114], OperandSize::Dword)
}

fn vmaxpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 189, 155, 95, 202], OperandSize::Qword)
}

fn vmaxpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectDisplaced(RBX, 129820086, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 173, 207, 95, 139, 182, 229, 188, 7], OperandSize::Qword)
}

fn vmaxpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 149, 210, 95, 52, 137], OperandSize::Qword)
}

