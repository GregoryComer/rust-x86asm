use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 168, 250], OperandSize::Dword)
}

fn vfmadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 168, 57], OperandSize::Dword)
}

fn vfmadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 168, 255], OperandSize::Qword)
}

fn vfmadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 168, 49], OperandSize::Qword)
}

fn vfmadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 168, 237], OperandSize::Dword)
}

fn vfmadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 1825217410, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 168, 140, 74, 130, 155, 202, 108], OperandSize::Dword)
}

fn vfmadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 168, 195], OperandSize::Qword)
}

fn vfmadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1684424207, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 168, 140, 138, 15, 70, 102, 100], OperandSize::Qword)
}

fn vfmadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 168, 228], OperandSize::Dword)
}

fn vfmadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1723396526, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 168, 140, 250, 174, 241, 184, 102], OperandSize::Dword)
}

fn vfmadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 76465182, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 159, 168, 131, 30, 196, 142, 4], OperandSize::Dword)
}

fn vfmadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 157, 132, 168, 205], OperandSize::Qword)
}

fn vfmadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1149401247, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 229, 138, 168, 28, 181, 159, 120, 130, 68], OperandSize::Qword)
}

fn vfmadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 221, 158, 168, 19], OperandSize::Qword)
}

fn vfmadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 168, 249], OperandSize::Dword)
}

fn vfmadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 649545648, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 168, 180, 255, 176, 71, 183, 38], OperandSize::Dword)
}

fn vfmadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 185, 168, 16], OperandSize::Dword)
}

fn vfmadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 141, 172, 168, 214], OperandSize::Qword)
}

fn vfmadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 171, 168, 20, 120], OperandSize::Qword)
}

fn vfmadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1880503547, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 157, 186, 168, 12, 189, 251, 52, 22, 112], OperandSize::Qword)
}

fn vfmadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 217, 168, 210], OperandSize::Dword)
}

fn vfmadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 113416469, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 204, 168, 140, 193, 21, 153, 194, 6], OperandSize::Dword)
}

fn vfmadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 221, 168, 52, 207], OperandSize::Dword)
}

fn vfmadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 133, 249, 168, 210], OperandSize::Qword)
}

fn vfmadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 237, 204, 168, 4, 153], OperandSize::Qword)
}

fn vfmadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 493854471, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 237, 209, 168, 12, 157, 7, 159, 111, 29], OperandSize::Qword)
}

