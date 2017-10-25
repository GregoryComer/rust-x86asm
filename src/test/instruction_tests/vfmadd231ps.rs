use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 184, 205], OperandSize::Dword)
}

fn vfmadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 184, 51], OperandSize::Dword)
}

fn vfmadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 184, 218], OperandSize::Qword)
}

fn vfmadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1646055240, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 184, 172, 203, 72, 207, 28, 98], OperandSize::Qword)
}

fn vfmadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 184, 197], OperandSize::Dword)
}

fn vfmadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDX, 1054048191, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 184, 130, 191, 127, 211, 62], OperandSize::Dword)
}

fn vfmadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 184, 219], OperandSize::Qword)
}

fn vfmadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RDX, 365806784, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 184, 162, 192, 196, 205, 21], OperandSize::Qword)
}

fn vfmadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 184, 254], OperandSize::Dword)
}

fn vfmadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 184, 58], OperandSize::Dword)
}

fn vfmadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 1833104053, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 155, 184, 139, 181, 242, 66, 109], OperandSize::Dword)
}

fn vfmadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 21, 135, 184, 202], OperandSize::Qword)
}

fn vfmadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 45, 137, 184, 36, 71], OperandSize::Qword)
}

fn vfmadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 37, 154, 184, 28, 211], OperandSize::Qword)
}

fn vfmadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 170, 184, 254], OperandSize::Dword)
}

fn vfmadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 22108247, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 171, 184, 170, 87, 88, 81, 1], OperandSize::Dword)
}

fn vfmadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EAX, 1862231484, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 189, 184, 184, 188, 101, 255, 110], OperandSize::Dword)
}

fn vfmadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 45, 171, 184, 252], OperandSize::Qword)
}

fn vfmadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 769405027, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 171, 184, 12, 117, 99, 48, 220, 45], OperandSize::Qword)
}

fn vfmadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 488248363, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 109, 181, 184, 148, 211, 43, 20, 26, 29], OperandSize::Qword)
}

fn vfmadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 221, 184, 225], OperandSize::Dword)
}

fn vfmadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 206, 184, 28, 255], OperandSize::Dword)
}

fn vfmadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1463008687, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 220, 184, 172, 146, 175, 189, 51, 87], OperandSize::Dword)
}

fn vfmadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 117, 221, 184, 203], OperandSize::Qword)
}

fn vfmadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectDisplaced(RAX, 1869853041, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 93, 198, 184, 160, 113, 177, 115, 111], OperandSize::Qword)
}

fn vfmadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1353196877, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 93, 220, 184, 60, 85, 77, 37, 168, 80], OperandSize::Qword)
}

